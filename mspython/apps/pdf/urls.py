from django.urls import path, include
from .views import ClienteNew

urlpatterns = [
    path('pdf/',ClienteNew.as_view(), name="pdf_view"),
]