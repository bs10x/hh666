import hashlib

def feistel_network_encrypt(text, key, rounds=8):
    block_size = len(text) // 2
    left, right = text[:block_size], text[block_size:]

    for _ in range(rounds):
        key_hash = int(hashlib.sha256(key.encode()).hexdigest(), 16)
        
        new_right = str(int(right, 16) ^ key_hash)
        new_right = new_right.zfill(len(right))
        
        left, right = right, hex(int(left, 16) ^ int(new_right, 16))[2:]

    return right + left

def feistel_network_decrypt_to_utf8(encrypted_text, key, rounds=8):
    block_size = len(encrypted_text) // 2
    left, right = encrypted_text[block_size:], encrypted_text[:block_size]

    for _ in range(rounds):
        key_hash = int(hashlib.sha256(key.encode()).hexdigest(), 16)

        new_left = str(int(left, 16) ^ key_hash)
        new_left = new_left.zfill(len(left))

        right, left = left, hex(int(right, 16) ^ int(new_left, 16))[2:]

    decrypted_hex = left + right
    decrypted_bytes = bytes.fromhex(decrypted_hex)
    # Replace invalid UTF-8 bytes with the Unicode replacement character
    decrypted_text = decrypted_bytes.decode('utf-8', errors='replace')
    # Replace the Unicode replacement character with an emoji
    emoji = "🔥"  # Change this to the emoji you want
    decrypted_text = decrypted_text.replace('\ufffd', emoji)
    return decrypted_text

# Example
original_text = "abc1abc1"
key = "secret_key"

# Encrypt
encrypted_text = feistel_network_encrypt(original_text, key)
print(f"Original text: {original_text}")
print(f"Encrypted text: {encrypted_text}")

# Decrypt and convert to UTF-8
decrypted_text_utf8 = feistel_network_decrypt_to_utf8(encrypted_text, key)
print(f"Decrypted text (UTF-8): {decrypted_text_utf8}")