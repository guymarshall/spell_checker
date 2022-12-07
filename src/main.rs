#![forbid(unsafe_code)]

// for each word in misspelledWords.txt, search whole directory. As soon as word is found, break the loop and print the word, then move to the next word until done
// for word in list
//     for file in directory
//         if word found
//             print word
//             break

fn main() {
    println!("Hello, world!");
}

/*
import os


directory = input("Enter directory to search: ")
# these two replace calls are for paths that are surrounded in quotes
directory = directory.replace("'", "")
directory = directory.replace("\"", "")

misspelled_words = []
with open("misspelledWords.txt", "r") as file:
    for line in file:
        misspelled_words.append(line)

for root, dirs, files in os.walk(directory):
    for file in files:
        try:
            with open(file, "r") as filereader:
                for line in filereader:
                    print(line)
        except FileNotFoundError:
            print("File not found.")
        # with open(os.path.join(root, file), "r") as auto:
        #     print(os.path.join(root, file))

# search_string = input("Enter string to search for: ")
# output = []

# with open(filename, "r") as file:
#     for line in file:
#         if search_string in line:
#             output.append(line)
#             # print(line)

# with open(f"{search_string}.txt", "w") as file:
#     for line in output:
#         file.write(line)



# with open(file_path, "r") as file:
#     file_contents = ""

#     for line in file:
#         file_contents += line

#     for word in html_base_words:
#         opening_tag = f"<{word}"
#         closing_tag = f"</{word}"

#         opening_count = file_contents.count(opening_tag)
#         if opening_count > 0:
#             counts[opening_tag] = opening_count

#         closing_count = file_contents.count(closing_tag)
#         if closing_count > 0:
#             counts[closing_tag] = closing_count

#     for character in programming_characters:
#         character_count = file_contents.count(character)
#         if character_count > 0:
#             counts[character] = character_count

# user_input = input("Press enter to close...")
*/