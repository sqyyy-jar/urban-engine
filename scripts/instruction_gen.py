import pprint

instructions = {
    'not': {'r': 2, 'i': 0, 'n': False},
    'add': {'r': 3, 'i': 0, 'n': False},
    'add_imm': {'r': 2, 'i': -1, 'n': True},
    'sub': {'r': 3, 'i': 0, 'n': False},
    'sub_imm': {'r': 2, 'i': -1, 'n': True},
    'mul': {'r': 3, 'i': 0, 'n': False},
    'mul_imm': {'r': 2, 'i': -1, 'n': True},
    'div': {'r': 3, 'i': 0, 'n': False},
    'div_imm': {'r': 2, 'i': -1, 'n': True},
    'adds': {'r': 3, 'i': 0, 'n': False},
    'adds_imm': {'r': 2, 'i': -1, 'n': True},
    'subs': {'r': 3, 'i': 0, 'n': False},
    'subs_imm': {'r': 2, 'i': -1, 'n': True},
    'muls': {'r': 3, 'i': 0, 'n': False},
    'muls_imm': {'r': 2, 'i': -1, 'n': True},
    'divs': {'r': 3, 'i': 0, 'n': False},
    'divs_imm': {'r': 2, 'i': -1, 'n': True},
    'addf': {'r': 3, 'i': 0, 'n': False},
    'subf': {'r': 3, 'i': 0, 'n': False},
    'mulf': {'r': 3, 'i': 0, 'n': False},
    'divf': {'r': 3, 'i': 0, 'n': False},
    'and': {'r': 3, 'i': 0, 'n': False},
    'or': {'r': 3, 'i': 0, 'n': False},
    'xor': {'r': 3, 'i': 0, 'n': False},
    'shl': {'r': 2, 'i': 6, 'n': True},
    'shr': {'r': 2, 'i': 6, 'n': True},
    'shrs': {'r': 2, 'i': 6, 'n': True},
    'ldr': {'r': 2, 'i': 0, 'n': False},
    'ldr_imm': {'r': 1, 'i': -1, 'n': True},
    'mov': {'r': 2, 'i': 0, 'n': False},
    'mov_imm': {'r': 1, 'i': -1, 'n': True},
    'movs_imm': {'r': 1, 'i': -1, 'n': True},
    'br': {'r': 1, 'i': 0, 'n': False},
    'b_imm': {'r': 0, 'i': -1, 'n': True},
    'brl': {'r': 1, 'i': 0, 'n': False},
    'bl_imm': {'r': 0, 'i': -1, 'n': True},
    'b.eq_imm': {'r': 1, 'i': -1, 'n': True},
    'b.nq_imm': {'r': 1, 'i': -1, 'n': True},
    'b.lt_imm': {'r': 1, 'i': -1, 'n': True},
    'b.gt_imm': {'r': 1, 'i': -1, 'n': True},
    'b.le_imm': {'r': 1, 'i': -1, 'n': True},
    'b.ge_imm': {'r': 1, 'i': -1, 'n': True},
    'nop': {'r': 0, 'i': 0, 'n': False},
    'halt': {'r': 0, 'i': 0, 'n': False},
    'syscall_imm': {'r': 0, 'i': 8, 'n': True},
}


def insn(key, value):
    space = 32
    registers = value['r']
    value_size = value['i']
    if value_size != -1:
        space -= value_size
    space -= registers * 5
    space -= 6
    return {
        'name': key,
        'space': space,
        'value-size': value_size,
        'registers': registers,
        'number-arg': value['n'],
    }


def scan():
    all = []
    for key, value in instructions.items():
        all.append(insn(key, value))
    return sorted(all, key=lambda it: it['name'])


def gen_insn(value, id):
    name = value['name']
    registers = value['registers']
    doc_name = name.replace('_imm', '')
    const_name = name.upper().replace('.', '_').replace('IMM', 'IMMEDIATE')
    res = f'/// {doc_name}'
    for i in range(registers):
        if i != 0:
            res += ','
        res += f' <X{i}>'
    if value['number-arg']:
        if registers > 0:
            res += ','
        res += f' <i{32 - 6 - registers * 5}>'
    res += '\npub const INSN_'
    res += const_name
    res += f': u32 = 0b{id:06b}_'
    for _ in range(32 - 6 - registers * 5):
        res += '0'
    for _ in range(registers):
        res += '_00000'
    res += ';\npub const ENDINSN_'
    res += const_name
    res += f': u32 = 0b{id:06b}_'
    for _ in range(32 - 6 - registers * 5):
        res += '1'
    for _ in range(registers):
        res += '_11111'
    res += ';'
    return res


res = scan()
for i, x in enumerate(res):
    print(gen_insn(x, i))
