from pathlib import Path
import argparse

MISSPELLED_WORDS_FILENAME: str = 'misspelled_words.txt'
OUTPUT_FILENAME: str = 'output.txt'

def save_file_contents(directory: Path) -> dict[str, str]:
    path_contents: dict[str, str] = {}
    if directory.is_dir():
        for entry in directory.iterdir():
            if entry.is_dir():
                path_contents.update(save_file_contents(entry))
            else:
                if entry.name == MISSPELLED_WORDS_FILENAME:
                    continue
                try:
                    with entry.open('r', encoding='utf-8') as file:
                        contents = file.read()
                except Exception:
                    contents = ''
                path_contents[str(entry)] = contents
    return path_contents

def main():
    argument_parser = argparse.ArgumentParser()
    argument_parser.add_argument('--path', dest='path', type=str, help='Add path')
    arguments = argument_parser.parse_args()
    path: str = arguments.path

    if not path:
        print('Path must be given via --path flag')
        exit(1)

    directory: Path = Path(path)
    print(f'Scanning directory "{directory}"')

    path_contents: dict[str, str] = save_file_contents(directory)

    try:
        with open(MISSPELLED_WORDS_FILENAME, 'r', encoding='utf-8') as file:
            misspelled_words = [line.strip() for line in file.readlines()]
    except FileNotFoundError:
        print(f'File "{MISSPELLED_WORDS_FILENAME}" not found.')
        return

    matched_words_by_file = {
        path: [word for word in misspelled_words if word in contents]
        for path, contents in path_contents.items()
        if any(word in contents for word in misspelled_words)
    }

    with open(OUTPUT_FILENAME, 'w', encoding='utf-8') as output_file:
        for path, words in matched_words_by_file.items():
            output_file.write(f'{path}: {words}\n')

if __name__ == '__main__':
    main()
