from pathlib import Path
import argparse
import re

MISSPELLED_WORDS_FILENAME: str = 'misspelled_words.txt'
OUTPUT_FILENAME: str = 'output.txt'

WORD_REGEX = re.compile(r'\b\w+\b')

def save_file_contents(directory: Path) -> dict[str, list[str]]:
    path_contents: dict[str, list[str]] = {}
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
                        words = WORD_REGEX.findall(contents)
                except Exception:
                    words = []
                path_contents[str(entry)] = words
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

    path_contents: dict[str, list[str]] = save_file_contents(directory)

    try:
        with open(MISSPELLED_WORDS_FILENAME, 'r', encoding='utf-8') as file:
            misspelled_words = [line.strip() for line in file.readlines()]
    except FileNotFoundError:
        print(f'File "{MISSPELLED_WORDS_FILENAME}" not found.')
        return

    misspelled_set = set(misspelled_words)

    matched_words_by_file = {
        path: sorted(set(word for word in words if word in misspelled_set))
        for path, words in path_contents.items()
        if any(word in misspelled_set for word in words)
    }

    with open(OUTPUT_FILENAME, 'w', encoding='utf-8') as output_file:
        for path, words in matched_words_by_file.items():
            output_file.write(f'{path}: {", ".join(words)}\n')

if __name__ == '__main__':
    main()
