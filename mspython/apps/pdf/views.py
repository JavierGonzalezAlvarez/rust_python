from django.shortcuts import render
from .forms import ClienteForm
from .models import Cliente
from django.urls import reverse_lazy
from django.views import generic
from django.contrib.messages.views import SuccessMessageMixin
from django.core.files.storage import FileSystemStorage
from django.http import HttpResponseRedirect
import os

class ClienteNew(SuccessMessageMixin, generic.CreateView):
    model=Cliente
    template_name="pdf/pdf.html"
    context_object_name = 'obj'
    form_class=ClienteForm    
    success_url= reverse_lazy("pdf:pdf_view")    
    success_message="Cliente Creado"
    error_message="Cliente no Creado"

    #para funciones
    '''    
    def upload(request):
        if request.method == 'POST' and request.FILES['img']:
            file = request.FILES['img']
            fss = FileSystemStorage()
            filename = fss.save(file.name, file)
            file_url = fss.url(filename)            
            return render(request, 'pdf/pdf.html', {'file_url': file_url})                            
        return render(request, 'pdf/pdf.html')
    '''

    def form_valid(self, form):        
        '''
            run rust
            use def form_valid() with CreateView to run the script
        ''' 
        self.object = form.save()        
        failure = os.system('./script.sh')        
        if failure:
            print("error en script")
        return HttpResponseRedirect(self.get_success_url())        
