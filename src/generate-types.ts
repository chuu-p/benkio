import * as fs from 'fs';
import { compileFromFile } from 'json-schema-to-typescript'

const SCHEMA_FOLDER = './shared/schemas';

fs.readdirSync(SCHEMA_FOLDER).forEach(file => {
  let source_file = `${SCHEMA_FOLDER}/${file}`;
  let target_file = `./src/models/${file.replace(".schema.json", ".d.ts")}`;
  console.log(`generating ${source_file} -> ${target_file}`);
  compileFromFile(`${SCHEMA_FOLDER}/${file}`)
    .then(ts => fs.writeFileSync(target_file, ts))
});
