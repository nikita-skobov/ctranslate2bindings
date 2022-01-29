# Ctranslate2 rust bindings

This library is a work in progress. It actually does not contain the full bindings to CTranslate2,
but rather just a subset of functionality that I needed at the time.

We depend on `libctranslate2.a` static library existing at the root of this repository. There is a provided
Dockerfile which will build this static library.

To create the `libctranslate2.a` you can do the following:

```sh
# this will take a while...
docker build -t translatebuilder .
id=$(docker create translatebuilder)
docker cp $id:/opt/CTranslate2/build/libctranslate2.a ./
docker rm -v $id
```

Once this is the root of your repo, you can build the library, and the example with:

```sh
cargo build --examples
```

The example will ask you to input some text, and then it will run it through CTranslate2 and output the translation.

To do this, it needs a pre-trained model, which you can find many here: https://www.argosopentech.com/argospm/index/

The files you download there have extension .argos, but really they are just zip files and can be extracted as such.

Once extracted, you only need 2 things from these pre-trained models:

1. the `model/` folder for the actual translation
2. The `sentencepiece.model` for tokenizing
