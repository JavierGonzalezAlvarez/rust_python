# rust, pdf and python (django)

## Crear entorno virtual
virtualenv entorno_virtual -p python3

## Activar entorno virtual
source entorno_virtual/bin/activate

## install requirements
pip install -r requirements.txt 

## crear proyecto
django-admin startproject apps

## crear app
apps/python3 manage.py startapp pdf

## crear BD
$ psql -d postgres -U javier
CREATE DATABASE rustpython WITH OWNER javier;

## migraciones
python3 manage.py makemigrations
python3 manage.py migrate

## crear usuario
python3 manage.py createsuperuser
javier
89_Lp%wD

## crear modelo
en pdf app