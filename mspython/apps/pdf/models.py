from django.db import models

# Create your models here.

class Cliente(models.Model):    
    name = models.CharField(
        max_length=15,         
        #unique=True,        
        blank=False,
        verbose_name="Nombre"
    )    
    surname = models.CharField(
        max_length=25,                 
        blank=False,
        verbose_name="Nombre"
    )    
    img = models.FileField(
        upload_to='./img',                     
        blank=True,
        verbose_name="img"
    )  
    fecha_alta = models.DateField(blank=True, null=True, verbose_name="Fecha alta")  
    pdf = models.FileField(
        upload_to='./pdf',                      
        blank=True,
        verbose_name="pdf"
    )    
            
    def __str__ (self):
        return '{}'.format(self.name)            
    
    def save(self):
        self.name = self.name.upper()
        super(Cliente, self).save()

    class Meta:
        verbose_name_plural = "Cliente"

