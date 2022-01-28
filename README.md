To create the `libctranslate2.a` you can do the following:

```sh
# this will take a while...
docker build -t translatebuilder .
id=$(docker create translatebuilder)
docker cp $id:/opt/CTranslate2/build/libctranslate2.a ./
docker rm -v $id
```
