# foo


class StructPadding(object):
    def __init__(self, index, ty):
        self.index = index
        self.ty = ty


class StructPaddedField(object):
    def __init__(self, field, paddings=[]):
        self.field = field
        self.paddings = paddings


def align(current_position, alignment):
    remainder = current_position % alignment
    if remainder > 0:
        return current_position - remainder + alignment
    return current_position


def generate_paddings(index, size):
    paddings = []

    if size % 2 == 1:
        paddings.append(StructPadding(index, 'u8'))
        index += 1
        size -= 1

    if size % 4 == 2:
        paddings.append(StructPadding(index, 'u16'))
        index += 1
        size -= 2

    if size % 8 == 4:
        paddings.append(StructPadding(index, 'u32'))
        index += 1
        size -= 4

    for i in range(size // 8):
        paddings.append(StructPadding(index + i, 'u64'))

    return paddings


def struct_padded_fields(ctx, object):
    fields = []
    padding_index = 0
    position = 0

    last_field = None
    for raw_field in sorted((object.Fields(i) for i in range(object.FieldsLength())), key=lambda f: f.Offset()):
        if last_field is not None:
            alignment = ctx.field_alignment(raw_field)
            padded = align(position, alignment)
            paddings = generate_paddings(padding_index, padded - position)
            fields.append(StructPaddedField(last_field, paddings))

            padding_index += len(paddings)
            position = padded + ctx.field_size(raw_field)
        else:
            position += ctx.field_size(raw_field)

        last_field = raw_field

    if last_field is not None:
        alignment = ctx.field_alignment(raw_field)
        padded = align(position, object.Minalign())
        paddings = generate_paddings(padding_index, padded - position)
        fields.append(StructPaddedField(last_field, paddings))

    return fields
