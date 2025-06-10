import * as borsh from "borsh";

export class counterAccount {
  count = 0;

  constructor({ count }: { count: number }) {
    this.count = count;
  }
}

export const schema: borsh.Schema = {
  struct: {
    count: "u32",
  },
};

export const COUNTER_SIZE = borsh.serialize(
  schema,
  new counterAccount({ count: 0 })
).length;

// console.log(borsh.serialize(schema, new counterAccount({ count: 1 })));
console.log(COUNTER_SIZE);
