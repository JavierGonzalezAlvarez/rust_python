from django.contrib import admin
from django.urls import path, include

urlpatterns = [
    path('admin/', admin.site.urls),
    path('cliente/', include(('pdf.urls', 'pdf'), namespace='pdf')),
]

admin.site.site_header = "PDF desde Rust"
admin.site.site_title = "PDF - Rust/Django"
admin.site.index_title = "Backend Admin"
