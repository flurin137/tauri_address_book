export interface Address {
    id: string,
    name: string,
    email: string,
    address: string,
    gender: Gender,
}

export enum Gender {
    Male = "Male",
    Female = "Female"
}

export function newAddress() : Address
{
    return {
        id: crypto.randomUUID(),
        name: "",
        email: "",
        address: "",
        gender: Gender.Female,
    }
}
