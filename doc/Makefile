local:
	@mkdir -p out
	xelatex -output-directory=out doc.tex
	biber -output-directory=out doc
	xelatex -output-directory=out doc.tex
	xelatex -output-directory=out doc.tex

docker:
	@mkdir -p out
	docker build -t emu-latex .
	docker run -u "$(shell id -u):$(shell id -g)" --rm -v ./out:/out emu-latex
clean:
	rm -rf out/*
