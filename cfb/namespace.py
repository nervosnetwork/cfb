from collections import OrderedDict


class Namespace(object):
    def __init__(self, name=None):
        self.name = name
        self.children = {}
        self.objects = OrderedDict()
        self.enums = OrderedDict()

    def has_definitions(self):
        return len(self.objects) > 0 or len(self.enums) > 0

    def append_object(self, o):
        name = o.Name().decode('utf-8')
        parts = name.split('.')

        container = self
        for namespace_name in parts[0:-1]:
            container = container.get_or_insert_child(namespace_name)

        container.objects[parts[-1]] = o

    def append_enum(self, e):
        name = e.Name().decode('utf-8')
        parts = name.split('.')

        container = self
        for namespace_name in parts[0:-1]:
            container = container.get_or_insert_child(namespace_name)

        container.enums[parts[-1]] = e

    def get_or_insert_child(self, name):
        if name in self.children:
            return self.children[name]
        else:
            child = Namespace(name)
            self.children[name] = child
            return child

    @staticmethod
    def from_schema(schema):
        root_namespace = Namespace()

        for i in range(schema.ObjectsLength()):
            root_namespace.append_object(schema.Objects(i))
        for i in range(schema.EnumsLength()):
            root_namespace.append_enum(schema.Enums(i))

        return root_namespace
