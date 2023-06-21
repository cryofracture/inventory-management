FROM python:3.6

RUN pip install flask
COPY requirements.txt /frontend/requirements.txt
COPY application.py /frontend/application.py
RUN pip install -r /frontend/requirements.txt

WORKDIR /frontend

CMD [ "python", "application.py" ]