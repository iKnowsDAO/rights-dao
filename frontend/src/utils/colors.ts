const tagColors = [
    'pink',
    'red',
    'orange',
    'green',
    'cyan',
    'blue',
    'purple',
    'magenta',
    'volcano',
    'gold',
    'lime',
    'geekblue',
];

const hashCode = (value: string): number => {
    let h = 0,
        off = 0;
    const length = value.length;
    for (let i = 0; i < length; i++) {
        h = 31 * h + value.charCodeAt(off++);
    }
    return h;
};
export const getTagColor = (tag: string) => tagColors[hashCode(tag) % tagColors.length];
