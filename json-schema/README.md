# JSON Schema Generator

A simple tool that generates Rust and TypeScript types from a JSON Schema.

- JSON Schema Definitions: https://jsontypedef.com/
- JSON:API Schema: https://jsonapi.org/

## Usage

```bash
npm ci --omit=dev
npm run start
```

## Environment Variables

| Variable          | Default                                             | Description                               |
| ----------------- | --------------------------------------------------- | ----------------------------------------- |
| INPUT_JTD_DIR     | `./src`                                             | The directory to search for JTD files.    |
| OUTPUT_RS_DIR     | `../server/workspace/lib-json-schema/src/generated` | The directory to output Rust files.       |
| OUTPUT_TS_DIR     | `../client/src/generated`                           | The directory to output TypeScript files. |
| JTD_EXTENSION     | `.jtd.json`                                         | The file extension to search for.         |
| JTD_EXECUTOR_PATH | `./.bin`                                            | The path to the JTD executor.             |
