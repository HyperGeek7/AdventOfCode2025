def load_input(file_name: str):
    with open(file_name, 'r') as fin:
        input_lines = [line.strip() for line in fin if len(line) > 0]
    
    return input_lines