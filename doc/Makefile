all:
	@mkdir -p out/out
	pdflatex -output-directory=out doc.tex
	biber -output-directory=out out/doc
	pdflatex -output-directory=out doc.tex
clean:
	rm -rf out/*
