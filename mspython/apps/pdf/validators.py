import os
from django.core.exceptions import ValidationError

def validar_extension(value):    
    extension = os.path.splitext(value.name)[1]
    extension_valida = ['.jpg','.png',]
    if not extension.lower() in extension_valida:
        raise ValidationError('Extensión no soportada')
