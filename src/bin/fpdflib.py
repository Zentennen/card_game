    pub fn text(&self, txt: &str, x: f64, y: f64) {
        let args = (x, y, txt);
        self.pdf.call_method1("text", args).unwrap();
    }

    pub fn write(&self, txt: &str) {
        let args = vec![("txt", txt)].into_py_dict(self.py);
        self.pdf.call_method("set_xy", (), Some(args)).unwrap();
    }

    pub fn cell(&self, txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        self.pdf.call_method1("cell", args).unwrap();
    }

    pub fn center_cell(&self, txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        let kwargs = [("align", "C")].into_py_dict(self.py);
        self.pdf.call_method("cell", args, Some(kwargs)).unwrap();
    }

    pub fn multi_cell(&self,txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        self.pdf.call_method1("multi_cell", args).unwrap();
    }

    pub fn center_multi_cell(&self, txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        let kwargs = [("align", "C")].into_py_dict(self.py);
        self.pdf.call_method("multi_cell", args, Some(kwargs)).unwrap();
    }

from fpdf import FPDF

def text(pdf: FPDF, x: f64, y: f64):