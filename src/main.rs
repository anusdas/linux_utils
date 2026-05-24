use std::env;
use std::path::Path;
use lopdf::{Document, Object};
use lopdf::dictionary; // Import the macro into our explicit scope
use std::collections::BTreeMap;

fn main() {
    // 1. Collect arguments from the command line
    let args: Vec<String> = env::args().collect();

    // We expect: [program, input1, input2, output]
    if args.len() < 4 {
        println!("❌ Error: Missing arguments!");
        println!("Usage: combine-pdf <input1.pdf> <input2.pdf> <output.pdf>");
        std::process::exit(1);
    }

    let input1 = &args[1];
    let input2 = &args[2];
    let output = &args[3];

    println!("🔄 Merging files in order:\n   1. {}\n   2. {}\n⏳ Processing...", input1, input2);

    // 2. Open and load both input documents safely
    let doc1 = match Document::load(Path::new(input1)) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Failed to open {}: {}", input1, e);
            std::process::exit(1);
        }
    };

    let doc2 = match Document::load(Path::new(input2)) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Failed to open {}: {}", input2, e);
            std::process::exit(1);
        }
    };

    // 3. Create a fresh target document template
    let mut output_doc = Document::with_version("1.5");
    let mut pages_to_add = BTreeMap::new();

    // Context helper closure to pull pages out without ID collisions
    let mut add_pages = |doc: Document| {
        let mut local_doc = doc;
        let max_id = output_doc.max_id;
        
        // Shift internal object IDs so documents don't overwrite each other
        local_doc.renumber_objects_with(max_id + 1);
        
        // FIX: Added '&' to borrow objects instead of moving/destroying them
        for (object_id, object) in &local_doc.objects {
            output_doc.objects.insert(*object_id, object.clone());
        }

        for (_, page_id) in local_doc.get_pages() {
            if let Ok(page_obj) = output_doc.get_object(page_id) {
                pages_to_add.insert(page_id, page_obj.clone());
            }
        }
        output_doc.max_id = max_id + local_doc.max_id;
    };

    // Run the documents sequentially to lock the user's explicit order
    add_pages(doc1);
    add_pages(doc2);

    // 4. Assemble the unified global PDF architecture tree
    let mut page_kids = Vec::new();
    for (page_id, _) in pages_to_add {
        page_kids.push(Object::Reference(page_id));
    }

    let count = page_kids.len() as i32;
    let pages_id = output_doc.new_object_id();
    
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => Object::Array(page_kids),
        "Count" => count,
    };
    output_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = output_doc.new_object_id();
    let catalog_dict = dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    };
    output_doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));
    output_doc.trailer.set("Root", Object::Reference(catalog_id));

    // 5. Save the compiled binary out to disk
    match output_doc.save(output) {
        Ok(_) => println!("✅ Success! Combined PDF saved to: {}", output),
        Err(e) => eprintln!("❌ Failed to write output file: {}", e),
    }
}
