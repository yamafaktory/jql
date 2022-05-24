| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.8 ± 0.7 | 31.6 | 35.3 | 16.84 ± 1.85 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.2 | 1.5 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.5 ± 0.6 | 31.4 | 34.5 | 17.65 ± 1.89 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.8 ± 0.2 | 1.5 | 2.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.6 ± 0.7 | 31.5 | 36.0 | 17.30 ± 1.99 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.9 ± 0.2 | 1.5 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 164.2 ± 0.7 | 163.0 | 169.2 | 1.33 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 123.3 ± 1.8 | 119.8 | 154.9 | 1.00 |

