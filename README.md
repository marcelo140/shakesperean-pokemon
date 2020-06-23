## How to run

```
docker build -t shakes-mon .
docker run --net=host shakes-mon
```

## Available endpoints

- /healthcheck
- /pokemon/{name}

## Environment variables

- *FUN_TRANSLATION_API_KEY*: The API key for FunTranslations API.
