import dayjs from 'dayjs';

export function formatDate(dateString) {
    const date = dayjs(dateString);
    // Then specify how you want your dates to be formatted
    var res = date.format('MMMM D, HH:mm:ss');
    if (res.includes('Invalid')) {
        return "Never"
    }
    else return res
}