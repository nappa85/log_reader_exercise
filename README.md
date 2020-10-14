# CLI tool to analyze logs

Spec: Write a CLI tool in rust which reads a file where each line is an arbitrary json object that includes a fiedl called `type`. I should output a table containing the number of objects with each `type`, and a total byte size of all the messages each `type`.

* You may use external libraries.

Score based on:
* how easy to use it is
* how fast it is
* code quality
* error handling
* Unit test will not be scored and are not necessary

Example input:
```json
{"type":"B","foo":"bar","items":["one","two"]}
{"type": "A","foo": 4.0 }
{"type": "B","bar": "abcd"}
```
