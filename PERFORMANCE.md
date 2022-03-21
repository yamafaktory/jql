| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 37.3 ± 2.0 | 33.9 | 55.5 | 18.62 ± 3.86 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.4 | 1.4 | 6.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 37.2 ± 2.1 | 33.7 | 49.9 | 18.42 ± 9.24 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 1.0 | 1.4 | 19.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 38.4 ± 3.0 | 33.9 | 69.6 | 18.38 ± 4.64 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.5 | 1.4 | 7.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 162.2 ± 8.3 | 152.2 | 298.2 | 1.31 ± 0.10 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 123.7 ± 7.3 | 112.8 | 219.9 | 1.00 |

