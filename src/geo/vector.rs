#[derive(Debug, Clone, Copy)]
pub struct Vector{
    pub magnitude: f64,
    pub direction: f64,
}
#[allow(dead_code)]
impl Vector {
    pub fn new(magnitude: f64, direction_in_degrees: f64) -> Self {
        Self {
            magnitude,
            direction: direction_in_degrees.to_radians(),
        }
    }

    pub fn x_component(&self) -> f64 {
        self.magnitude * self.direction.cos()
    }

    pub fn y_component(&self) -> f64 {
        self.magnitude * self.direction.sin()
    }

    pub fn scale(&mut self, factor: f64) {
        self.magnitude *= factor;
    }
    
    pub fn add(&self, other: &Self) -> Self {
        
        let x1 = self.x_component();
        let y1 = self.y_component();
        let x2 = other.x_component();
        let y2 = other.y_component();

        let x = x1 + x2;
        let y = y1 + y2;

        let magnitude = (x.powi(2) + y.powi(2)).sqrt();
        let direction = y.atan2(x).to_degrees();

        Self::new(magnitude, direction)
    }    

    pub fn subtract(&self, other: &Self) -> Self {
    todo!("subtract vectors")
    // WWWWWWWWWWWWWWWNKOOkOKNWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWWWNKOOkOKNWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWNk:..   .,lx0WWWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWNk:..   .,lx0WWWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWMWd.       .. ,0WWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWMWd.       .. ,0WWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWMWXl.'...      .xWWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWMWXl.'...      .xWWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWNkodc;,,;. ..lNWWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWNkodc;,,;. ..lNWWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWMWWXkolc:,.lKKNMMWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWMWWXkolc:,.lKKNMMWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWMWWXkolc:,.lKKNMMWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWMWWXkolc:,.lKKNMMWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWWWWKxlc;:OMMWWWWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWWWWKxlc;:OMMWWWWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWWWWWWNKxooox0NNWWWWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWWWWWWNKxooox0NNWWWWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWWWN0d:...   .okxOXWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWWWN0d:...   .okxOXWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWXOo;.  .     .;,...;ox0NWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWXOo;.  .     .;,...;ox0NWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWNOd:'. ..''.    ..,;,'... .,cdKWWWWWWWWWWWWWW \\         // WWWWWWWWNOd:'. ..''.    ..,;,'... .,cdKWWWWWWWWWWWWWW \\
    // WWWWWWXx,.  ..',;'....   .,;;:::;'...,xKXNWWWWWWWWWWW \\         // WWWWWWXx,.  ..',;'....   .,;;:::;'...,xKXNWWWWWWWWWWW \\
    // WWWWWN0x:,',;;;;'.        .,colodl;;oKXK0KXWWWWWWWWWW \\         // WWWWWN0x:,',;;;;'.        .,colodl;;oKXK0KXWWWWWWWWWW \\
    // WWWWNXXNXx:,,;;'.         .,lxdoko;lKWWWWWWWWWWWWWWWW \\    -    // WWWWNXXNXx:,,;;'.         .,lxdoko;lKWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWNx:;;,.           .:llol:l0WWWWWWWWWWWWWWWWW \\         // WWWWWWWWWNx:;;,.           .:llol:l0WWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWXd;;;.           .;;;;;l0WWWWWWWWMWWWWWWWWW \\         // WWWWWWWWWWXd;;;.           .;;;;;l0WWWWWWWWMWWWWWWWWW \\
    // WWWWWWWWWWWKl;,.           .';;;cOWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWKl;,.           .';;;cOWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWNx;.     ..      .,;:kNWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWNx;.     ..      .,;:kNWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWW0c.     ''      .';oXWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWW0c.     ''      .';oXWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWNd,.   .,,.     .,cOWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWNd,.   .,,.     .,cOWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWk;. ..,;;,..  .';dNWMWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWk;. ..,;;,..  .';dNWMWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWW0:...,;;;;,'. .':OWWWWWWWWWWWKddkXWWWWWW \\         // WWWWWWWWWWWWW0:...,;;;;,'. .':OWWWWWWWWWWWKddkXWWWWWW \\
    // WWWWWWWWWWWWMXc..';;;;;;;'...lKWWWWWWWWWWKdlldOKNWWWW \\         // WWWWWWWWWWWWMXc..';;;;;;;'...lKWWWWWWWWWWKdlldOKNWWWW \\
    // WWWWWWWWWWWWWXc..,clooc;;,...oNWWWWWWWWWWNXXXXKXNWWWW \\         // WWWWWWWWWWWWWXc..,clooc;;,...oNWWWWWWWWWWNXXXXKXNWWWW \\
    // WWWWWWWWWNKko:..l0XNNNKOxdc..oNWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWNKko:..l0XNNNKOxdc..oNWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWKOkOO0NWWWWWWWWWK: ;KWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWKOkOO0NWWWWWWWWWK: ;KWWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWWWWWWWWWWWWWWW0l:kNWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWWWWWWWWWWWWWWW0l:kNWWWWWWWWWWWWWWWWWWWWW \\
    // WWWWWWWWWWWWWWWWWWWWWMWWNWWWWNNWWWWWWWWWWWWWWWWWWWWWW \\         // WWWWWWWWWWWWWWWWWWWWWMWWNWWWWNNWWWWWWWWWWWWWWWWWWWWWW \\
    }
}

