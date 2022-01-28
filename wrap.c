#include <string.h>
#include <iostream>
#include <ctranslate2/translator_pool.h>

extern "C" {
    typedef struct FfiTranslator FfiTranslator;
    typedef struct {
        char **m_data;
        int m_size;
    } FfiTranslateResult;

    FfiTranslator *ct2_translator_new(const char *model_path)
    {
        try {
            return (FfiTranslator *) new ctranslate2::Translator(model_path, ctranslate2::Device::CPU);
        } catch (...) {
            return nullptr;
        }
    }

    void ct2_translator_free(FfiTranslator *t)
    {
        if (t == nullptr) {
            return;
        }
        ctranslate2::Translator *translator = ((ctranslate2::Translator *)t);
        delete translator;
    }

    FfiTranslateResult ct2_translator_translate(FfiTranslator *t, const char **input, int count)
    {
        std::size_t countsize = (std::size_t) count;
        std::vector<std::string> translate_this;
        std::copy_n (input, countsize, std::back_inserter (translate_this));
        ctranslate2::Translator *translator = ((ctranslate2::Translator *) t);
        // for (const auto& s1 : translate_this) {
        //     std::cout << s1 << std::endl;
        // }
        ctranslate2::TranslationResult result = translator->translate(translate_this);
        // for (const auto& token : result.output()) {
        //     std::cout << token << ' ';
        // }
        // std::cout << std::endl;

        std::vector<std::string> output = result.output();
        char **str_array;
        str_array = (char **) malloc(output.size() * sizeof(char *));
        for (int i = 0; i < output.size(); i += 1) {
            std::string si = output[i];
            str_array[i] = (char *) malloc((si.length() + 1) * sizeof(char));
            memcpy(str_array[i], si.data(), si.length());
            str_array[i][si.length()] = 0;
        }
        FfiTranslateResult translateresult;
        translateresult.m_data = str_array;
        translateresult.m_size = (int) output.size();
        return translateresult;
    }
}
