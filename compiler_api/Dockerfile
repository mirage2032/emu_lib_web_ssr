FROM z88dk/z88dk

# Install Python
RUN apk add --no-cache python3 py3-pip

WORKDIR /web

# Create Virtual Environment
RUN python3 -m venv venv
RUN venv/bin/pip install --upgrade pip

# Install PIP Dependencies
COPY requirements.txt .
RUN venv/bin/pip install -r requirements.txt

COPY api.py .

# Expose Port
EXPOSE 4560

# Run
CMD ["venv/bin/fastapi","run", "--port", "4560" ,"api.py"]