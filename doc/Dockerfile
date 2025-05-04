FROM alpine

RUN apk update
RUN apk add --no-cache texlive
RUN apk add --no-cache perl texlive-langextra biber
RUN apk add --no-cache texmf-dist-latexextra
RUN apk add --no-cache texmf-dist-bibtexextra
RUN apk add --no-cache texmf-dist-fontsextra
RUN apk add --no-cache texmf-dist-fontsrecommended
RUN apk add --no-cache texlive-xetex
RUN apk add --no-cache font-jetbrains-mono

WORKDIR /doc
RUN mkdir /out

COPY . .

CMD xelatex --shell-escape -output-directory=/out doc.tex && \
	biber -output-directory=/out doc && \
	xelatex -output-directory=/out doc.tex && \
	xelatex -output-directory=/out doc.tex
