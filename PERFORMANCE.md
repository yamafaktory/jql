| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.7 ± 0.7 | 31.5 | 35.7 | 16.20 ± 5.00 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.6 | 1.6 | 11.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.5 ± 0.6 | 31.4 | 35.2 | 17.22 ± 1.82 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.9 ± 0.2 | 1.6 | 3.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.5 ± 0.6 | 31.4 | 34.6 | 18.18 ± 2.52 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.8 ± 0.2 | 1.5 | 5.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 164.5 ± 0.7 | 163.1 | 173.5 | 1.27 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 129.6 ± 2.1 | 125.0 | 157.5 | 1.00 |

