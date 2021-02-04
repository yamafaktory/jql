| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 43.3 ± 2.1 | 39.1 | 55.7 | 21.80 ± 6.42 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.6 | 1.4 | 9.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 43.7 ± 2.6 | 38.2 | 69.8 | 19.33 ± 8.02 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.3 ± 0.9 | 1.5 | 21.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 43.9 ± 3.0 | 38.3 | 77.7 | 22.76 ± 6.33 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.9 ± 0.5 | 1.3 | 7.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 195.5 ± 9.7 | 171.4 | 251.7 | 1.24 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 158.2 ± 8.9 | 135.8 | 207.5 | 1.00 |

