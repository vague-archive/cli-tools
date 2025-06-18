# Image Compressor

Recursively compresses all images to the proper gpu format in a directory, using a config file to orchestrate the compression. The repo includes an example config. You can specify the config path with -c or --config. See example:

`image_compressor compress -c /path/to/config/directory/config.json`

```json
{
    "from_directory": "/path/to/assets",
    "to_directory": "/built/path/for/assets",
    "ignore_list": [
        "favicon.png",
        "character"
    ],
    "number_of_threads": 8,
    "compression_container": "KTX",
    "skip_errors": true,
    "verbose": true,
    "compression_config": {
        "config_type": "BasisUniversalBasisLZETC1s",
        "config": {
            "BasisUniversalBasisLZETC1s": {
                "thread_count": 4,
                "compression_level": 4
            }
        },
        "premultiply": true
    }
}
```
