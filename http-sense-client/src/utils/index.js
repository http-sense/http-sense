export const getColorCode = status => {
  switch (`${status}`[0]) {
    case '2':
      return '#2ed573';
    case '3':
      return '#3742fa';
    case '4':
    case '5':
      return '#ff4757';
    default:
      return '#000';
  }
};
