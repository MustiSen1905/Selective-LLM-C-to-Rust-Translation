"""Standalone driver: re-runs translate_unsafe_to_safe on tiny-aes
without requiring c2rust / gcc / graph analysis. Uses a hand-written
function order. Verifies Rule 20 (PRESERVE FUNCTION SIGNATURE)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from entities.FunctionObject import FunctionObject
from translation.translation import main as translate_unsafe_to_safe

# Topological order, leaves first.
# (getSBoxValue, getSBoxInvert, Multiply were inlined by c2rust.)
NAMES = [
    "xtime",
    "KeyExpansion",
    "AddRoundKey", "SubBytes", "ShiftRows", "MixColumns",
    "InvSubBytes", "InvShiftRows", "InvMixColumns",
    "Cipher", "InvCipher",
    "AES_init_ctx", "AES_init_ctx_iv", "AES_ctx_set_iv",
    "XorWithIv",
    "AES_ECB_encrypt", "AES_ECB_decrypt",
    "AES_CBC_encrypt_buffer", "AES_CBC_decrypt_buffer",
    "AES_CTR_xcrypt_buffer",
]

order = [FunctionObject(n, "aes.c") for n in NAMES]
translate_unsafe_to_safe("Rust-Outcome/tiny-AES-c-master", order)
