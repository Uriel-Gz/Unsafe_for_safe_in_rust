
class Token:
    def __init__(self, type_, value):
        self.type = type_
        self.value = value

    def __repr__(self):
        return f'Token({self.type}, {self.value})'

class Lexer:
    keywords = [
                'unsafe', 'let', 'mut', 'as', 'Some',
                'Ok', 'Box', 'return','trait', 'enum',           # 'read', 'write','from_raw', 'into_raw',
                'extern', 'fn','struct', 'impl', 
                'match', 'if', 'else', 'for', 'while'            #'ptr' es un caso especial por llevar el mismo nombre que el modulo ptr
                ]

    def __init__(self, text):
        self.text = text
        self.pos = 0
        self.current_char = self.text[0] if self.text else None

    def advance(self):
        self.pos += 1
        if self.pos > len(self.text) - 1:
            self.current_char = None
        else:
            self.current_char = self.text[self.pos]

    def skip_whitespace(self):
        while self.current_char and self.current_char.isspace():
            self.advance()

    def read_identifier(self):
        result = ''
        while self.current_char and (self.current_char.isalnum() or self.current_char == '_'):
            result += self.current_char
            self.advance()
        return result

    def read_string_literal(self):
        result = ''
        self.advance()  # skip opening quote
        while self.current_char and self.current_char != '"':
            result += self.current_char
            self.advance()
        self.advance()  # skip closing quote
        return result

    def tokenize(self):
        tokens = []
        while self.current_char is not None:
            if self.current_char.isspace():
                self.skip_whitespace()
                continue
            elif self.current_char.isalpha() or self.current_char == '_':
                ident = self.read_identifier()
                # Keywords
                if ident in self.keywords:
                    tokens.append(Token('KEYWORD', ident))
                else:
                    tokens.append(Token('IDENTIFIER', ident))
            elif self.current_char == '"':
                string_val = self.read_string_literal()
                tokens.append(Token('STRING_LITERAL', string_val))
            elif self.current_char.isdigit():
                num = ''
                while self.current_char and self.current_char.isdigit():
                    num += self.current_char
                    self.advance()
                tokens.append(Token('NUMBER', num))
            elif self.current_char == '*':
                tokens.append(Token('ASTERISK', '*'))
                self.advance()
            elif self.current_char == '&':
                tokens.append(Token('AMPERSAND', '&'))
                self.advance()
            elif self.current_char == '=':
                tokens.append(Token('EQUALS', '='))
                self.advance()
            elif self.current_char == '.':
                tokens.append(Token('DOT', '.'))
                self.advance()
            elif self.current_char == '(':
                tokens.append(Token('LPAREN', '('))
                self.advance()
            elif self.current_char == ')':
                tokens.append(Token('RPAREN', ')'))
                self.advance()
            elif self.current_char == '{':
                tokens.append(Token('LBRACE', '{'))
                self.advance()
            elif self.current_char == '}':
                tokens.append(Token('RBRACE', '}'))
                self.advance()
            elif self.current_char == ';':
                tokens.append(Token('SEMICOLON', ';'))
                self.advance()
            elif self.current_char == ',':
                tokens.append(Token('COMMA', ','))
                self.advance()
            elif self.current_char == ':':
                tokens.append(Token('COLON', ':'))
                self.advance()
            elif self.current_char == '<':
                tokens.append(Token('LT', '<'))
                self.advance()
            elif self.current_char == '>':
                tokens.append(Token('GT', '>'))
                self.advance()
            elif self.current_char == '+':
                tokens.append(Token('PLUS', '+'))
                self.advance()
            elif self.current_char == '-':
                tokens.append(Token('MINUS', '-'))
                self.advance()
            elif self.current_char == '/':
                tokens.append(Token('SLASH', '/'))
                self.advance()
            else:
                # Unknown character, skip or raise error
                self.advance()
        return tokens
