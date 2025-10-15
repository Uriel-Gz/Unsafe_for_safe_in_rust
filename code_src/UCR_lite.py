import extractor
import modifier

def menu():
    print("Seleccione una opción:")
    print("1. Extraer código unsafe")
    print("2. Reemplazar código unsafe")
    print("3. Salir")
    choice = input("Ingrese su elección (1/2/3): ")
    return choice


if __name__ == "__main__":
    while True:
        menu_choice = menu()
        if menu_choice == '1':
            origin = input("Ingrese la ruta del archivo Rust: ")
            extractor.extract_unsafe_code(origin)
        elif menu_choice == '2':
            origin = input("Ingrese la ruta del archivo Rust: ")
            modifier.replace_unsafe_code(origin)
        elif menu_choice == '3':
            print("Saliendo del programa.")
            break
        else:
            print("Opción inválida. Por favor, intente de nuevo.")
