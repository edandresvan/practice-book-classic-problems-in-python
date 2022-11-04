from unbreakable_encryption import encrypt, decrypt


def main():
  key1, key2 = encrypt("One Time Pad!")
  result: str = decrypt(key1, key2)
  print(result)
  
if __name__ == "__main__":
  main()
