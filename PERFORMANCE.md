| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 37.9 ± 2.7 | 33.7 | 50.7 | 18.45 ± 5.51 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.1 ± 0.6 | 1.2 | 10.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.1 ± 2.6 | 34.0 | 49.3 | 19.67 ± 4.89 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.5 | 1.3 | 7.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 40.7 ± 3.3 | 34.4 | 54.0 | 20.44 ± 5.36 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.5 | 1.2 | 9.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 173.7 ± 12.8 | 152.1 | 234.0 | 1.28 ± 0.13 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 135.6 ± 9.4 | 116.2 | 179.7 | 1.00 |

