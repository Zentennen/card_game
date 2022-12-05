from fpdf import FPDF

pdf = FPDF()
pdf.add_page()
pdf.set_font("Helvetica", size = 12)
pdf.text_mode = "STROKE"
pdf.cell(txt = "HELLO WORLD!")
pdf.output("test.pdf")