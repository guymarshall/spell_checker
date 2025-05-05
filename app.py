from pathlib import Path

MISSPELLED_WORDS_FILENAME: str = 'misspelled_words.txt'

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
    print('Example path: "c:/users/YOUR_USERNAME/downloads"')

    try:
        with open(MISSPELLED_WORDS_FILENAME, 'r', encoding='utf-8') as f:
            misspelled_words = [line.strip() for line in f.readlines()]
    except FileNotFoundError:
        print('Invalid path.')
        return

    path: str = input('Enter path to search: ').strip()
    directory: Path = Path(path)
    path_contents: dict[str, str] = save_file_contents(directory)

    matched_words_by_file = {
        path: [word for word in misspelled_words if word in contents]
        for path, contents in path_contents.items()
        if any(word in contents for word in misspelled_words)
    }

    for path, words in matched_words_by_file.items():
        print(f'{path}: {', '.join(words)}')

if __name__ == '__main__':
    main()
