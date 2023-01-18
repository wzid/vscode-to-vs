use std::{env, fs, path::Path, vec};
use aho_corasick::AhoCorasick;
use code_file::CodeFile;

use crate::code_file::FileType;

pub mod guid;
pub mod code_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check if any arguments are passed to the program
    if args.len() < 3 {
        println!("Error: Not enough arguments specified");
        return;
    }

    let project_id = guid::generate_guid();
    let solution_id = guid::generate_guid();

    //Get project name
    let project_name = &args[1];

    // Convert the argument to a Path object
    let folder_path = Path::new(&args[2]);

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            println!("{:?}", entry.file_name());
        }
    }

    let new_folder_path = folder_path.join("Visual Studio");
    if new_folder_path.exists() {
        // Delete the old folder if it exists
        fs::remove_dir_all(&new_folder_path)
            .expect("Failed to delete old visual studio folder and contents");
    }
    
    //Create the Visual Studio folder
    fs::create_dir(&new_folder_path).expect("Failed to create Visual Studio folder");
    
    //We now need a project folder which all the code will be in along with everything but the solution file
    let project_path = new_folder_path.join(project_name);
    
    fs::create_dir(&project_path).expect("Failed to create project folder");

    let mut code_files: Vec<CodeFile> = vec![];

    //Copy over all of the code/data files
    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_file() {

            let code_file_type = match entry.path().extension().unwrap().to_str() {
                Some("cpp") => FileType::SOURCE,
                Some("h") => FileType::HEADER,
                Some("dat") => FileType::TEXT,
                Some("txt") => FileType::TEXT,
                _ => FileType::HEADER
            };

            code_files.push(CodeFile {
                file_name: entry.file_name().to_str().unwrap().to_string(),
                file_type: code_file_type
            });

            let new_path = &project_path.join(entry.file_name());
            fs::copy(entry.path(), new_path).expect("Failed to copy code/data files");
        }
    }

    //Read the solution file contents
    let mut solution_contents = fs::read_to_string("assets/sln").expect("Failed to read the file");

    //Here we implement something similar to https://docs.rs/aho-corasick/latest/aho_corasick/struct.AhoCorasick.html#examples
    let patterns = &["NAME", "PROJECTID", "SOLUTIONID"];
    let replace_with = &[&project_name, &project_id, &solution_id];
    let ac = AhoCorasick::new(patterns);

    //Using the AhoCorasick crate this only allocates one string
    solution_contents = ac.replace_all(&solution_contents, replace_with);

    let new_solution_file = format!("{}.sln", project_name);
    fs::write(&new_folder_path.join(new_solution_file), solution_contents)
        .expect("Failed to write to file");

    let user_contents = fs::read_to_string("assets/vcxproj.user")
        .expect("Failed to read the file");

    let new_user_file = format!("{}.vcxproj.user", project_name);
    fs::write(&project_path.join(new_user_file), user_contents).expect("Failed to write to file");

    let mut vcxproj_first_contents = fs::read_to_string("assets/vcxproj").expect("Failed to read the file");
    vcxproj_first_contents = vcxproj_first_contents.replace("PROJECTID", &project_id);

    append_second_part_vcxproj(&code_files, &mut vcxproj_first_contents);

    let new_vcxproj_file = format!("{}.vcxproj", project_name);
    fs::write(&project_path.join(new_vcxproj_file), vcxproj_first_contents).expect("Failed to write to file");

    let mut filter_first_contents = fs::read_to_string("assets/vcxproj.filters").expect("Failed to read the file");
    append_second_part_filter(&code_files, &mut filter_first_contents);

    let new_filter_file = format!("{}.vcxproj.filters", project_name);
    fs::write(&project_path.join(new_filter_file), filter_first_contents).expect("Failed to write to file");


    
}

fn append_second_part_filter(code_files: &Vec<CodeFile>, first_part: &mut String) {
    let item_group = String::from("\n  <ItemGroup>");
    let mut compile = item_group.clone();
    let mut text = item_group.clone();
    let mut header = item_group.clone();

    for file in code_files {
        match file.file_type {
            FileType::SOURCE => {
                compile.push('\n');
                compile.push_str(&format!("    <ClCompile Include=\"{}\">\n      <Filter>Source Files</Filter>\n    </ClCompile>", file.file_name));
            },
            FileType::TEXT => {
                text.push('\n');
                text.push_str(&format!("    <Text Include=\"{}\">\n      <Filter>Source Files</Filter>\n    </Text>", file.file_name));
            }
            FileType::HEADER => {
                header.push('\n');
                header.push_str(&format!("    <ClInclude Include=\"{}\">\n      <Filter>Header Files</Filter>\n    </ClInclude>", file.file_name));
            }
        }
    }

    if compile != item_group {
        compile.push_str("\n  </ItemGroup>");
        first_part.push_str(&compile);
    }

    if text != item_group {
        text.push_str("\n  </ItemGroup>");
        first_part.push_str(&text);
    }

    if header != item_group {
        header.push_str("\n  </ItemGroup>");
        first_part.push_str(&header);
    }

    first_part.push_str("\n</Project>");
}

fn append_second_part_vcxproj(code_files: &Vec<CodeFile>, first_part: &mut String) {
    let item_group = String::from("\n  <ItemGroup>");
    let mut compile = item_group.clone();
    let mut text = item_group.clone();
    let mut header = item_group.clone();

    for file in code_files {
        match file.file_type {
            FileType::SOURCE => {
                compile.push('\n');
                compile.push_str(&format!("    <ClCompile Include=\"{}\" />", file.file_name));
            },
            FileType::TEXT => {
                text.push('\n');
                text.push_str(&format!("    <Text Include=\"{}\" />", file.file_name));
            }
            FileType::HEADER => {
                header.push('\n');
                header.push_str(&format!("    <ClInclude Include=\"{}\" />", file.file_name));
            }
        }
    }

    if compile != item_group {
        compile.push_str("\n  </ItemGroup>");
        first_part.push_str(&compile);
    }

    if text != item_group {
        text.push_str("\n  </ItemGroup>");
        first_part.push_str(&text);
    }

    if header != item_group {
        header.push_str("\n  </ItemGroup>");
        first_part.push_str(&header);
    }

    first_part.push_str("\n  <Import Project=\"$(VCTargetsPath)\\Microsoft.Cpp.targets\" />\n  <ImportGroup Label=\"ExtensionTargets\">\n  </ImportGroup>\n</Project>");
}