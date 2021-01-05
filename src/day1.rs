/*
  Day 1 - fix expense report

  Part 1: Given a puzzle input of numbers, find the two entries that sum to 2020 and multiply them together

  Part 2: Find the three entries that sum to 2020 and multiply them together
*/

pub fn read_expenses() -> Vec<i32> {
  let v = vec![1046, 1565, 1179, 1889, 1683, 1837, 1973, 1584, 1581, 192, 1857, 1373, 1715, 1473, 1770, 1907, 1918, 1909, 1880, 1903, 1835, 1887, 1511, 1844, 1628, 1688, 1545, 1469, 1620, 1751, 1893, 1861, 511, 1201, 1641, 1874, 1946, 1701, 1777, 1829, 1609, 1805, 1678, 1928, 1398, 1555, 1675, 1798, 1485, 1911, 1974, 1663, 1919, 1635, 195, 1441, 1525, 1490, 1151, 1406, 1408, 1095, 1085, 1097, 1976, 1987, 1498, 1753, 1603, 1933, 1729, 1106, 1929, 1832, 1744, 1914, 1643, 1571, 1391, 1953, 1790, 1797, 1938, 258, 1957, 1858, 1506, 628, 1109, 1113, 1768, 1649, 1669, 694, 1803, 1849, 1395, 1754, 1421, 1575, 1632, 1998, 1693, 1499, 1550, 1771, 1902, 1801, 1549, 1459, 1826, 1927, 1507, 1718, 647, 1922, 1432, 1625, 1904, 1691, 1427, 1519, 1949, 1514, 1749, 1616, 1898, 1696, 1917, 1661, 1787, 1440, 1796, 1560, 1956, 1823, 1815, 1557, 1730, 1951, 1548, 1527, 1881, 1727, 1530, 1460, 1360, 1583, 1662, 1954, 1890, 1855, 1752, 1935, 1601, 1767, 1812, 1990, 1445, 1908, 2001, 1544, 1814, 1634, 1532, 1788, 1521, 1638, 1470, 1524, 1394, 1674, 1314, 1588, 1429, 1745, 1416, 1637, 1942, 484, 1467, 1764, 1743, 1401, 1471, 1458, 1335, 1866, 1399, 1393, 1708, 1694, 1447, 1972, 1478, 1182, 1672, 1813, 1546, 1535];
  return v;
}

pub fn find_expense(expenses: &Vec<i32>) -> i32 {
  let max = expenses.len();
  let mut i = 0;
  while i < max {
    let mut j = i+1;
    while j < max {
      if expenses[i] + expenses[j] == 2020 {
        return expenses[i] * expenses[j];
      }
      j += 1;
    }
    i +=1;
  }
  return 0;
}

pub fn find_expense_three(expenses: &Vec<i32>) -> i32 {
  let max = expenses.len();
  let mut i = 0;
  while i < max - 2 {
    let mut j = i+1;
    while j < max - 1 {
      let mut k = j+1;
      while k < max {
        if expenses[i] + expenses[j] + expenses[k] == 2020 {
          return expenses[i] * expenses[j] * expenses[k];
        }
        k += 1;
      }
      j += 1;
    }
    i +=1;
  }
  return 0;
}