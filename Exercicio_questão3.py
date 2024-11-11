import re #Expressoes regulares

# A expressão Resultante
pattern = r"-?\d+|['\"][\w]+['\"]|-?\d+:-?\d*|['\"][\w]+['\"]:['\"][\w]+['\"]"


# Testes
test_cases = [
    "x[0]",         # Número inteiro
    "x[-2]",        # Número negativo
    "x['Date']",    # Nome em aspas simples
    'x["Column"]',  # Nome em aspas duplas
    "x[0:5]",       # Slice de numeros
    "x['Data':'State']"  # Slice de strings
]

for test in test_cases:
    # Expressão para verificar se o termo é valido
    match = re.findall(pattern, test)
    if match:
        print(f"{test} Valido")
    else:
        print(f"{test} Invalido")
