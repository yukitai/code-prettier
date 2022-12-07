# Code-Prettier

## Language supports
  - rust
  - json
## How to make it support another language
  - enter `highlighting`
  - add {language-name}.json and write the highlighting rules in it
  - add extname map in language_map.json
    - add {extname}:{language-name} in the key `highlighter_map`
  - for more information, you can read the source code
