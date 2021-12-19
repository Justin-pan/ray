#[derive(Debug, PartialEq)]
pub struct Matrix44f32
{
    data: [f32; 16],
}

impl std::ops::Index<usize> for Matrix44f32
{
    type Output = [f32];
    fn index(&self, row: usize) -> &[f32]
    {
        let start = 4 * row;
        &self.data[start .. start + 4]
    }
}

impl std::ops::IndexMut<usize> for Matrix44f32
{
    fn index_mut(&mut self, row: usize) -> &mut [f32]
    {
        let start = 4 * row;
        &mut self.data[start .. start + 4]
    }
}

impl Matrix44f32
{
    pub fn new(mat: [f32; 16]) -> Matrix44f32
    {
        Matrix44f32 { data: mat }
    }

    pub fn identity() -> Matrix44f32
    {
        Matrix44f32
        {
            data: [1f32, 0f32, 0f32, 0f32,
                   0f32, 1f32, 0f32, 0f32,
                   0f32, 0f32, 1f32, 0f32,
                   0f32, 0f32, 0f32, 1f32]
        }
    }

    pub fn zeroes() -> Matrix44f32
    {
        Matrix44f32
        {
            data: [0f32; 16]
        }
    }

    pub fn product(&self, m2: &Matrix44f32) -> Matrix44f32
    {
        let mut ret = Matrix44f32::zeroes();
        for i in 0 .. 4
        {
            for j in 0 .. 4
            {
                ret[i][j] = self[i][0] * m2[0][j] +
                    self[i][1] * m2[1][j] +
                    self[i][2] * m2[2][j] +
                    self[i][3] * m2[3][j];
            }
        }
        ret
    }

    pub fn transpose(&self) -> Matrix44f32
    {
        let mut ret = Matrix44f32::zeroes();
        for i in 0 .. 4
        {
            for j in 0 .. 4
            {
                ret[i][j] = self[j][i]
            }
        }
        ret
    }

    pub fn inverse(&self) -> Matrix44f32
    {
        let mut inv = Matrix44f32::zeroes();
        inv.data[0] = self.data[5]  * self.data[10] * self.data[15] -
                    self.data[5]  * self.data[11] * self.data[14] -
                    self.data[9]  * self.data[6]  * self.data[15] +
                    self.data[9]  * self.data[7]  * self.data[14] +
                    self.data[13] * self.data[6]  * self.data[11] -
                    self.data[13] * self.data[7]  * self.data[10];

        inv.data[4] = -self.data[4] * self.data[10] * self.data[15] +
                    self.data[4]  * self.data[11] * self.data[14] +
                    self.data[8]  * self.data[6]  * self.data[15] -
                    self.data[8]  * self.data[7]  * self.data[14] -
                    self.data[12] * self.data[6]  * self.data[11] +
                    self.data[12] * self.data[7]  * self.data[10];

        inv.data[8] = self.data[4]  * self.data[9] * self.data[15] -
                    self.data[4]  * self.data[11] * self.data[13] -
                    self.data[8]  * self.data[5] * self.data[15] +
                    self.data[8]  * self.data[7] * self.data[13] +
                    self.data[12] * self.data[5] * self.data[11] -
                    self.data[12] * self.data[7] * self.data[9];

        inv.data[12] = -self.data[4] * self.data[9] * self.data[14] +
                    self.data[4]  * self.data[10] * self.data[13] +
                    self.data[8]  * self.data[5] * self.data[14] -
                    self.data[8]  * self.data[6] * self.data[13] -
                    self.data[12] * self.data[5] * self.data[10] +
                    self.data[12] * self.data[6] * self.data[9];

        inv.data[1] = -self.data[1] * self.data[10] * self.data[15] +
                    self.data[1]  * self.data[11] * self.data[14] +
                    self.data[9]  * self.data[2] * self.data[15] -
                    self.data[9]  * self.data[3] * self.data[14] -
                    self.data[13] * self.data[2] * self.data[11] +
                    self.data[13] * self.data[3] * self.data[10];

        inv.data[5] = self.data[0]  * self.data[10] * self.data[15] -
                    self.data[0]  * self.data[11] * self.data[14] -
                    self.data[8]  * self.data[2] * self.data[15] +
                    self.data[8]  * self.data[3] * self.data[14] +
                    self.data[12] * self.data[2] * self.data[11] -
                    self.data[12] * self.data[3] * self.data[10];

        inv.data[9] = -self.data[0] * self.data[9] * self.data[15] +
                    self.data[0]  * self.data[11] * self.data[13] +
                    self.data[8]  * self.data[1] * self.data[15] -
                    self.data[8]  * self.data[3] * self.data[13] -
                    self.data[12] * self.data[1] * self.data[11] +
                    self.data[12] * self.data[3] * self.data[9];

        inv.data[13] = self.data[0]  * self.data[9] * self.data[14] -
                    self.data[0]  * self.data[10] * self.data[13] -
                    self.data[8]  * self.data[1] * self.data[14] +
                    self.data[8]  * self.data[2] * self.data[13] +
                    self.data[12] * self.data[1] * self.data[10] -
                    self.data[12] * self.data[2] * self.data[9];

        inv.data[2] = self.data[1]  * self.data[6] * self.data[15] -
                    self.data[1]  * self.data[7] * self.data[14] -
                    self.data[5]  * self.data[2] * self.data[15] +
                    self.data[5]  * self.data[3] * self.data[14] +
                    self.data[13] * self.data[2] * self.data[7] -
                    self.data[13] * self.data[3] * self.data[6];

        inv.data[6] = -self.data[0] * self.data[6] * self.data[15] +
                    self.data[0]  * self.data[7] * self.data[14] +
                    self.data[4]  * self.data[2] * self.data[15] -
                    self.data[4]  * self.data[3] * self.data[14] -
                    self.data[12] * self.data[2] * self.data[7] +
                    self.data[12] * self.data[3] * self.data[6];

        inv.data[10] = self.data[0]  * self.data[5] * self.data[15] -
                    self.data[0]  * self.data[7] * self.data[13] -
                    self.data[4]  * self.data[1] * self.data[15] +
                    self.data[4]  * self.data[3] * self.data[13] +
                    self.data[12] * self.data[1] * self.data[7] -
                    self.data[12] * self.data[3] * self.data[5];

        inv.data[14] = -self.data[0] * self.data[5] * self.data[14] +
                    self.data[0]  * self.data[6] * self.data[13] +
                    self.data[4]  * self.data[1] * self.data[14] -
                    self.data[4]  * self.data[2] * self.data[13] -
                    self.data[12] * self.data[1] * self.data[6] +
                    self.data[12] * self.data[2] * self.data[5];

        inv.data[3] = -self.data[1] * self.data[6] * self.data[11] +
                    self.data[1]  * self.data[7] * self.data[10] +
                    self.data[5]  * self.data[2] * self.data[11] -
                    self.data[5]  * self.data[3] * self.data[10] -
                    self.data[9]  * self.data[2] * self.data[7] +
                    self.data[9]  * self.data[3] * self.data[6];

        inv.data[7] = self.data[0] * self.data[6] * self.data[11] -
                    self.data[0] * self.data[7] * self.data[10] -
                    self.data[4] * self.data[2] * self.data[11] +
                    self.data[4] * self.data[3] * self.data[10] +
                    self.data[8] * self.data[2] * self.data[7] -
                    self.data[8] * self.data[3] * self.data[6];

        inv.data[11] = -self.data[0] * self.data[5] * self.data[11] +
                    self.data[0]  * self.data[7] * self.data[9] +
                    self.data[4]  * self.data[1] * self.data[11] -
                    self.data[4]  * self.data[3] * self.data[9] -
                    self.data[8]  * self.data[1] * self.data[7] +
                    self.data[8]  * self.data[3] * self.data[5];

        inv.data[15] = self.data[0] * self.data[5] * self.data[10] -
                    self.data[0] * self.data[6] * self.data[9] -
                    self.data[4] * self.data[1] * self.data[10] +
                    self.data[4] * self.data[2] * self.data[9] +
                    self.data[8] * self.data[1] * self.data[6] -
                    self.data[8] * self.data[2] * self.data[5];

        let mut det = self.data[0] * inv.data[0] + self.data[1] * inv.data[4] +
                    self.data[2] * inv.data[8] + self.data[3] * inv.data[12];
        if det == 0f32
        {
            inv.data.iter_mut().for_each(|x| *x = 0f32);
        }
        else
        {
            det = 1f32 / det;
            for x in &mut inv.data
            {
                *x *= det;
            }
        }

        inv
    }
}
