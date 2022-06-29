export interface Address {
    id: string,
    name: string,
    email: string,
    address: string,
    gender: Gender,
}

enum Gender {
    Male,
    Female
}
