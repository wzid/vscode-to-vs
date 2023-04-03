use aho_corasick::AhoCorasick;
use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::{fs, path::PathBuf, vec};

pub mod code_file;
pub mod guid;
use code_file::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The name of the project to create
    project_name: String,
    /// the folder to create the project in
    folder_path: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let project_id = guid::generate_guid();
    let solution_id = guid::generate_guid();

    //Get project name
    let project_name = &args.project_name;

    // Convert the argument to a Path object
    let folder_path = args.folder_path.as_path();

    let new_folder_path = folder_path.join(project_name);
    if new_folder_path.exists() {
        // Delete the old folder if it exists
        fs::remove_dir_all(&new_folder_path)
            .context("Failed to delete old visual studio folder and contents")?;
    }

    //Create the Visual Studio folder
    fs::create_dir(&new_folder_path).context("Failed to create Visual Studio folder")?;

    //We now need a project folder which all the code will be in along with everything but the solution file
    let project_path = new_folder_path.join(project_name);

    fs::create_dir(&project_path).context("Failed to create project folder")?;

    //Create a vector to hold data on each code or data file in the original folder
    let mut code_files: Vec<CodeFile> = vec![];

    //Copy over all of the code/data files
    for entry in fs::read_dir(folder_path)?.filter_map(|x| x.ok()) {
        if entry.path().is_file() {
            let code_file_type = match entry.path().extension().and_then(|x| x.to_str()) {
                Some("cpp") => FileType::SOURCE,
                Some("h") => FileType::HEADER,
                Some("dat") => FileType::TEXT,
                Some("txt") => FileType::TEXT,
                _ => continue,
            };

            println!("Included {:?}", entry.file_name());

            code_files.push(CodeFile {
                file_name: entry.file_name().to_str().unwrap().to_string(),
                file_type: code_file_type,
            });

            let new_path = &project_path.join(entry.file_name());
            fs::copy(entry.path(), new_path).context("Failed to copy code/data files")?;
        }
    }

    let mut solution_contents = include_str!("../assets/sln");

    //Here we implement something similar to https://docs.rs/aho-corasick/latest/aho_corasick/struct.AhoCorasick.html#examples
    let patterns = &["NAME", "PROJECTID", "SOLUTIONID"];
    let replace_with = &[project_name, &project_id, &solution_id];
    let ac = AhoCorasick::new(patterns);

    //Using the AhoCorasick crate this only allocates one string
    let binding = ac.replace_all(solution_contents, replace_with);
    solution_contents = &binding;

    let new_solution_file = format!("{project_name}.sln");
    fs::write(new_folder_path.join(new_solution_file), solution_contents)
        .context("Failed to write to file")?;

    let user_contents = include_str!("../assets/vcxproj.user");

    let new_user_file = format!("{project_name}.vcxproj.user");
    fs::write(project_path.join(new_user_file), user_contents)
        .context("Failed to write to file")?;

    let mut vcxproj_first_contents = String::from(include_str!("../assets/vcxproj"));
    //Replace PROJECTID with the actual project id
    vcxproj_first_contents = vcxproj_first_contents.replace("PROJECTID", &project_id);

    // Use the code_files vector to append onto the first part of the vcxproj file
    append_second_part_vcxproj(&code_files, &mut vcxproj_first_contents);

    let new_vcxproj_file = format!("{project_name}.vcxproj");
    fs::write(project_path.join(new_vcxproj_file), vcxproj_first_contents)
        .context("Failed to write to file")?;

    let mut filter_first_contents = String::from(include_str!("../assets/vcxproj.filters"));
    // Use the code_files vector to append onto the first part of the vcxproj.filters file
    append_second_part_filter(&code_files, &mut filter_first_contents);

    let new_filter_file = format!("{project_name}.vcxproj.filters");
    fs::write(project_path.join(new_filter_file), filter_first_contents)
        .context("Failed to write to file")?;

    println!("\nSuccessfully created the Visual studio files 💖");

    Ok(())
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
            }
            FileType::TEXT => {
                text.push('\n');
                text.push_str(&format!(
                    "    <Text Include=\"{}\">\n      <Filter>Source Files</Filter>\n    </Text>",
                    file.file_name
                ));
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
            }
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
