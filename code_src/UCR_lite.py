import extractor
import modifier

def menu():
    print("\nSeleccione una opción:\n")
    print(f"\33[33m(1) \33[0mExtraer código unsafe")
    print(f"\33[93m(2) \33[0mReemplazar código unsafe")
    print(f"\33[91m(3) \33[0mSalir\n")
    choice = input("Ingrese su elección: ")
    return choice

def showinit():
    print("\033c", end="")  # Clear terminal
    print("\33[94m             ========================================== \33[0m")
    print("\33[94m         ====\33[92m      #     #     #####    ####  \33[94m         ====           \33[0m")
    print("\33[94m     ====\33[92m          #     #    #         #   #  \33[94m            ====       \33[0m")
    print("\33[94m ====\33[92m              #     #   #          ####  \33[94m                 ====   \33[0m")
    print("\33[94m ====\33[92m              #     #    #         #   #  \33[94m                ====   \33[0m")
    print("\33[94m     ====\33[92m           #####      #####    #   #\33[94m            ====       \33[0m")
    print("\33[94m         ====\33[97m                                 lite \33[94m    ====           \33[0m")
    print("\33[94m             ========================================== \33[0m")
    print(f"\n<< \33[92mAnalizador\33[97m, \33[92mextractor \33[92my \33[92mmodificador \33[97mde código unsafe en Rust\33[0m >>\n")

if __name__ == "__main__":
    showinit()
    while True:
        menu_choice = menu()
        if menu_choice == '1':
            origin = input("Ingrese la ruta del archivo Rust: ")
            extractor.extract_unsafe_code(origin)
        elif menu_choice == '2':
            origin = input("Ingrese la ruta del archivo Rust: ")
            modifier.replace_unsafe_code(origin)
        elif menu_choice == '3':
            print("\nSaliendo del programa.")
            break
        else:
            print("Opción inválida. Por favor, intente de nuevo.")
