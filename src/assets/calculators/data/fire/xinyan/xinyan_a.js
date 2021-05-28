import { rowsAir, tablePhysical } from "../../../utils";
import { getAttribute } from "@util/attribute";


let rowsA = [
    {
        key: "dmg1",
        chs: "普攻1段",
    },
    {
        key: "dmg2",
        chs: "普攻2段",
    },
    {
        key: "dmg3",
        chs: "普攻3段",
    },
    {
        key: "dmg4",
        chs: "普攻4段",
    },
];

let rowsB = [
    {
        key: "bDmg1",
        chs: "重击循环伤害",
    },
    {
        key: "bDmg2",
        chs: "重击终结伤害",
    },
];


export default function (artifacts, configObject, enemy) {
    let c = configObject.character;
    let w = configObject.weapon;
    let attribute = getAttribute(artifacts, c, w, configObject.buffs, configObject.artifactsConfig);

    let a = tablePhysical(attribute, configObject, enemy, rowsA, "a");
    let air = tablePhysical(attribute, configObject, enemy, rowsAir, "air");

    if (c.constellation >= 6) {
        attribute.attackStatic += 0.5 * attribute.defend();
    }

    let b = tablePhysical(attribute, configObject, enemy, rowsB, "b");

    return {
        a, b, air
    };
}