# hh666

## TODO:
+  split codebase into modules
    +  p_generator
    +  CLI (s_vec, k–>s2p_map)
    +  utility (settings, hardcoded variables)
    +  fpe4txt2ncipher (format preserving encryption for plaintext)
    +  img_generator

+  read input string as utf8 unicode points and load a vector with *only* the symbols present
    +   create unicode point counter variable, if s_vec > 720 -> Err



TEXT -> txt2dcipher -> txt2dcrypt -> txt2img -> IMAGE

IMAGE-> img2txt -> txt2dcrypt -> txt2dcipher -> TEXT


abc -> FPE(abc) -> hh666(FPE(abc)) -> 



fn ncipher FPE(txt2ncipher) -> returns ciphertext that has no more recognizable patterns

fn dcipher anti-FPE(txt2dcipher) -> returns the plaintext message

fn ncrypt hh666(txt2ncrypt) -> returns 